"""mitmproxy addon: capture only HTTP flows whose host matches a regex.

Configured via mitmdump's --set mechanism:

    mitmdump -s scope_addon.py \
             --set scope_out=/tmp/cap.flow \
             --set scope_regex='example\\.com'

This addon is a substitute for `--save-stream-file`. It writes matched flows
itself using mitmproxy's `io.FlowWriter`. Doing the writing ourselves is the
only way to enforce a regex scope: mitmproxy's built-in save_stream_file has
no regex filter, and its `save_stream_filter` option uses mitmproxy's own
FFilter DSL (e.g. `~d example.com`), not the regex our CLI exposes.

If `scope_regex` is empty or unset, all HTTP flows are written (the same
behavior as `--save-stream-file` with no filter).
"""

from __future__ import annotations

import re
from typing import BinaryIO

from mitmproxy import ctx, http, io


class ScopeCapture:
    def __init__(self) -> None:
        self._re: re.Pattern[str] | None = None
        self._writer: io.FlowWriter | None = None
        self._fh: BinaryIO | None = None

    def load(self, loader) -> None:  # type: ignore[no-untyped-def]
        loader.add_option(
            name="scope_regex",
            typespec=str,
            default="",
            help="Only flows whose request host matches this regex are "
            "written to scope_out. Empty = write everything.",
        )
        loader.add_option(
            name="scope_out",
            typespec=str,
            default="",
            help="Output .flow file path. Mirrors --save-stream-file.",
        )

    def configure(self, updated) -> None:  # type: ignore[no-untyped-def]
        if "scope_regex" in updated:
            pattern = ctx.options.scope_regex or ""
            if pattern:
                try:
                    self._re = re.compile(pattern)
                except re.error as e:
                    raise ValueError(f"invalid scope_regex {pattern!r}: {e}") from e
            else:
                self._re = None

        if "scope_out" in updated:
            self._close()
            out = ctx.options.scope_out
            if not out:
                return
            # Append-binary mode; FlowWriter uses tnetstring framing which
            # is append-safe (each record is self-delimiting).
            self._fh = open(out, "ab")  # noqa: SIM115
            self._writer = io.FlowWriter(self._fh)

    def done(self) -> None:
        self._close()

    def _close(self) -> None:
        if self._fh is not None:
            try:
                self._fh.flush()
                self._fh.close()
            except OSError:
                pass
        self._fh = None
        self._writer = None

    def _matches(self, host: str) -> bool:
        return self._re is None or bool(self._re.search(host))

    def _write(self, flow: http.HTTPFlow) -> None:
        if self._writer is None:
            return
        try:
            self._writer.add(flow)
        except OSError as e:
            # Best-effort; downstream tools re-emit a clear error if the file
            # ends up truncated.
            print(f"scope_addon: failed to write flow: {e}")  # noqa: T201

    # ---- HTTP hooks ------------------------------------------------------

    def request(self, flow: http.HTTPFlow) -> None:
        host = flow.request.pretty_host
        if not self._matches(host):
            # Drop: prevent upstream connection AND skip the response/error
            # write below by marking the flow as already-handled.
            flow.kill()
            flow.metadata["scope_skip"] = True

    def response(self, flow: http.HTTPFlow) -> None:
        if flow.websocket is not None:
            return  # websocket_end handles websocket flows
        if flow.metadata.get("scope_skip"):
            return
        if not self._matches(flow.request.pretty_host):
            return
        self._write(flow)

    def error(self, flow: http.HTTPFlow) -> None:
        # Mirror save.py: errors piggyback on response writes.
        self.response(flow)  # type: ignore[arg-type]


addons = [ScopeCapture()]

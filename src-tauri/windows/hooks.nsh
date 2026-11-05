; ! The versions named "Twitch Drops & Points Miner" were installed under that
; name, in their own folder: an update to "Twitch Miner" lands beside them, so
; their silent uninstaller removes the old copy, its shortcuts and its keys.
; It never deletes the data (that needs its box ticked, which /S leaves off),
; and the data folder moves to the new name on the app's first start.
!macro NSIS_HOOK_POSTINSTALL
  ReadRegStr $R9 SHCTX "Software\Microsoft\Windows\CurrentVersion\Uninstall\Twitch Drops & Points Miner" "UninstallString"
  ${If} $R9 != ""
    ExecWait '$R9 /S'
  ${EndIf}
!macroend

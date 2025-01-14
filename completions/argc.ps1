# Powershell completion for argc

$_argcCompletion = {
    param($wordToComplete, $commandAst, $cursorPosition)
    $argcfile = $(argc --argc-argcfile 2>$null)
    if (!$argcfile) {
        return;
    }
    if ($wordToComplete) {
        $cmds = $commandAst.CommandElements[1..($commandAst.CommandElements.Count - 2)]
    } else {
        $cmds = $commandAst.CommandElements[1..($commandAst.CommandElements.Count - 1)]
    }
    (argc --argc-compgen "$argcfile" $cmds 2>$null) -split " " | 
        Where-Object { $_ -like "$wordToComplete*" } |
        ForEach-Object { 
            if ($_.StartsWith("-")) {
                $t = [System.Management.Automation.CompletionResultType]::ParameterName
            } else {
                $t = [System.Management.Automation.CompletionResultType]::ParameterValue
            }
            [System.Management.Automation.CompletionResult]::new($_, $_, $t, '-')
        }
}

Register-ArgumentCompleter -Native -ScriptBlock $_argcCompletion -CommandName argc
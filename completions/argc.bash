# Bash completion for argc

_argc() {
    local argcfile cur opts
    cur="${COMP_WORDS[COMP_CWORD]}"
    COMPREPLY=()
    argcfile=$(argc --argc-argcfile 2>/dev/null)
    if [ $? != 0 ]; then
        return 0
    fi
    opts=$(argc --argc-compgen "$argcfile" ${COMP_WORDS[@]:1:$((${#COMP_WORDS[@]} - 2))} 2>/dev/null)
    COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
    return 0
}

complete -F _argc -o bashdefault -o default argc

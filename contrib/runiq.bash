_runiq() {
	local i cur prev opts cmds
	COMPREPLY=()
	cur="${COMP_WORDS[COMP_CWORD]}"
	prev="${COMP_WORDS[COMP_CWORD-1]}"
	cmd=""
	opts=""

	for i in ${COMP_WORDS[@]}
	do
		case "${i}" in
			runiq)
				cmd="runiq"
				;;

			*)
				;;
		esac
	done

	# The case statement checking the command name could simply be replaced with an if statement,
	# but this way makes it easier if one ultimately wants to handle completions for multiple commands within a single file.

	case "${cmd}" in
		runiq)
			# opts="-h --help -i --invert -s --statistics -V --version -f --filter"
			# Don't offer to complete short options for which equivalent long options exist.
			opts="--help --invert --statistics --version --filter"
			if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
				COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
				return 0
			fi
			case "${prev}" in

				-h | --help | -V | --version)
					return 0 # No other arguments matter if any of these is passed, so don't bother offering completions for any more unless the user tries to complete from another '-'
					;;
				-i | --invert)
					COMPREPLY=($(compgen -f ${cur}))
					return 0
					;;
				-s | --statistics)
					COMPREPLY=($(compgen -f ${cur}))
					return 0
					;;
				-f|--filter)
					COMPREPLY=($(compgen -W "sorted bloom naive digest" -- ${cur}))
					return 0
					;;
				--)
					_filedir
					return 0
					;;
				*)
					;;
			esac
			COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
			return 0
			;;
		# Don't consider any other cases.
	esac
}
complete -F _runiq -o bashdefault -o default runiq

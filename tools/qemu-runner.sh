#! /bin/sh

tmp=$( mktemp -d -p $PWD -t qemuXXXXXX )

if [ -z "${tmp}" ]
then
	echo "Could not create temporary directory." >&2
	exit 1
fi

trap "rm -rf \"${tmp}\"" EXIT HUP INT QUIT TERM

mkfifo "${tmp}/qemu.in" "${tmp}/qemu.out"

if [ ! -p "${tmp}/qemu.in" -o ! -p "${tmp}/qemu.out" ]
then
	echo "Could not create FIFOs." >&2
	exit 1
fi

(
  sleep 1
  printf "system_reset\ncont\n" > "${tmp}/qemu.in"
) &

child=$!

qemu-system-arm -monitor "pipe:${tmp}/qemu" -S "$@"

kill -1 $! 2>/dev/null
wait

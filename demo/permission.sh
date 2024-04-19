#!/bin/sh
for role in */*/; do
	chmod go-rwx $role/.gnupg -R # 也许要把kbx的权限加点？懒。
done

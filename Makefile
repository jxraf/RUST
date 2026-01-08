GIT = git

add:
	$(GIT) add .

commit:
	$(GIT) commit -m "$(m)"

push:
	$(GIT) push

all: add commit push

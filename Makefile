REGISTRY ?= localhost:32000
REGISTRY_PATH ?= $(REGISTRY)/

build: hello world greeting

hello:
	DOCKER_BUILDKIT=1 docker build -t $(REGISTRY_PATH)hello-world/hello -f hello.Dockerfile .
.PHONY: hello

world:
	DOCKER_BUILDKIT=1 docker build -t $(REGISTRY_PATH)hello-world/world -f world.Dockerfile .
.PHONY: world

greeting:
	DOCKER_BUILDKIT=1 docker build -t $(REGISTRY_PATH)hello-world/greeting -f greeting.Dockerfile .
.PHONY: greeter

push: push-hello push-world push-greeting
.PHONY: push

push-hello:
	docker push $(REGISTRY_PATH)hello-world/hello:latest
.PHONY: hello

push-world:
	docker push $(REGISTRY_PATH)hello-world/world:latest
.PHONY: world

push-greeting:
	docker push $(REGISTRY_PATH)hello-world/greeting:latest
.PHONY: greeting

skaffold:
	skaffold dev --default-repo $(REGISTRY)

install:
	helm install --namespace hello-world --create-namespace hello-world ./chart

uninstall:
	helm uninstall hello-world

REGISTRY ?= localhost:32000
REGISTRY_PATH ?= $(REGISTRY)/

build: hello world greeting api ui

ui:
	DOCKER_BUILDKIT=1 docker build -t $(REGISTRY_PATH)hello-world/ui -f ui.Dockerfile .
.PHONY: ui

hello:
	DOCKER_BUILDKIT=1 docker build -t $(REGISTRY_PATH)hello-world/hello -f hello.Dockerfile .
.PHONY: hello

world:
	DOCKER_BUILDKIT=1 docker build -t $(REGISTRY_PATH)hello-world/world -f world.Dockerfile .
.PHONY: world

greeting:
	DOCKER_BUILDKIT=1 docker build -t $(REGISTRY_PATH)hello-world/greeting -f greeting.Dockerfile .
.PHONY: greeting

api:
	DOCKER_BUILDKIT=1 docker build -t $(REGISTRY_PATH)hello-world/api -f api.Dockerfile .
.PHONY: api

push: push-hello push-world push-greeting push-api push-ui
.PHONY: push

push-ui:
	docker push $(REGISTRY_PATH)hello-world/ui:latest
.PHONY: push-ui

push-hello:
	docker push $(REGISTRY_PATH)hello-world/hello:latest
.PHONY: push-hello

push-world:
	docker push $(REGISTRY_PATH)hello-world/world:latest
.PHONY: push-world

push-greeting:
	docker push $(REGISTRY_PATH)hello-world/greeting:latest
.PHONY: push-greeting

push-api:
	docker push $(REGISTRY_PATH)hello-world/api:latest
.PHONY: push-api

skaffold:
	skaffold dev --default-repo $(REGISTRY)
.PHONY: skaffold

install:
	helm install --namespace hello-world --create-namespace hello-world ./chart
.PHONY: install

uninstall:
	helm uninstall hello-world
.PHONY: uninstall

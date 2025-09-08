set dotenv-load

default:
	@just --list --unsorted --color=always


run:
	COLOUR=blue cargo run --release

clean:
	cargo clean

build:
	docker build . --build-arg COLOUR=blue -t blue-green:blue
	docker build . --build-arg COLOUR=green -t blue-green:green
	docker build . --build-arg COLOUR=orange -t blue-green:blue-green
	minikube image load blue-green:blue
	minikube image load blue-green:green
	minikube image load blue-green:blue-green


run-images:
	docker run -d --rm --name blue -p 8080:8080 adrynov/blue-green:blue
	docker run -d --rm --name green -p 8081:8080 adrynov/blue-green:green
	docker run -d --rm --name orange -p 8082:8080 adrynov/blue-green:orange
	docker run -d --rm --name purple -p 8083:8080 adrynov/blue-green:purple
	docker run -d --rm --name red -p 8084:8080 adrynov/blue-green:red
	docker run -d --rm --name pink -p 8085:8080 adrynov/blue-green:pink


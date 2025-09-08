# Blue-Green Deployment Example

Updated code for the LinkedIn Learning course [Kubernetes Essential Training: Application Development](https://www.linkedin.com/learning/kubernetes-essential-training-application-development)

Serves either a blue or a green webpage on port 8080. Useful for testing blue/green deployment mechanisms, or, deploying onto machines that you otherwise couldn't tell apart.

## Pre-requisites

Install [Minikube](https://minikube.sigs.k8s.io/docs/start/) and [kubectl](https://kubernetes.io/docs/tasks/tools/install-kubectl/).

Install [Docker](https://docs.docker.com/get-docker/) or [Podman](https://podman.io/getting-started/installation).

Install [Rust](https://www.rust-lang.org/tools/install).

Install just tool with `cargo install just`.


## Build and Run

Build Docker images with `just build`.

Start minikube with `minikube start`.

Now follow the course instructions to deploy the application.


## References

Forked from https://github.com/mt-inside/blue-green as the original code did not work on Intel/AMD architectures.

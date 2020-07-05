# Learn Rust Web


## Local

Install Minikube

https://github.com/kubernetes/minikube

1. check vm

```bash
sysctl -a | grep -E --color 'machdep.cpu.features|VMX'
```

2. install minikube

```bash
brew install minikube
```

3. start minikube

```bash
minikube start
```

4. install kubectl

```bash
brew install kubernetes-cli
```

5. check kubectl

```bash
kubectl cluster-info
```




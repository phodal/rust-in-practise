

1. build

```bash
docker build -t hello-world .
```

2. run in docker

```bash
docker run --name hello-world -p 8080:8080 -d hello-world
```

3. run in k8s

```
kubectl run hello-world --image=hello-world --port=8080 --generator=run-pod/v1
```

4. expose

```
kubectl expose rc hello-world --type=LoadBalancer --name=hello-http
```

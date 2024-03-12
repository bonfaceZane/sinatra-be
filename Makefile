
#surrealDB
surreal-backup:

dev:
	cargo run --bin=backend


# kubernates
pods:
	kubectl get pod
service:
	kubectl get service
config:
	kubectl apply -f mongo-configmap.yaml
mongo-express:
	kubectl apply -f mongo-express.yaml

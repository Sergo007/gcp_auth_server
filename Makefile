IMAGE_LOCATION=gcr.io/ecare21-dev
IMAGE_NAME=stock_math_api
IMAGE_TAG=0.1.6

docker-build:
	docker build --target runtime -t ${IMAGE_LOCATION}/${IMAGE_NAME}:${IMAGE_TAG} -f Dockerfile .

docker-build-debian:
	docker build -t ${IMAGE_LOCATION}/${IMAGE_NAME}:${IMAGE_TAG} -f debian/Dockerfile .

docker-push:
	docker push ${IMAGE_LOCATION}/${IMAGE_NAME}:${IMAGE_TAG}

docker-run:
	docker run -p 8080:8080 ${IMAGE_LOCATION}/${IMAGE_NAME}:${IMAGE_TAG}
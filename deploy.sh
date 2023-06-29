# dist=dist

# proj=$1
# [ $proj ] || { echo "Specify a GCP project as a first arg"; exit 1; }

proj=ecare21-dev
service=stock_math_api
image=gcr.io/$proj/$service

gcloud builds submit --target runtime --tag "$image":latest src --project=$proj &&
gcloud beta run deploy $service \
  --image="$image":latest \
  --ingress=all \
  --no-allow-unauthenticated \
  --memory=256Mi \
  --platform=managed \
  --region=us-central1 \
  --project=$proj


# gcloud beta run deploy $service \
#   --image="$image":latest \
#   --service-account=$service@$proj.iam.gserviceaccount.com \
#   --ingress=all \
#   --no-allow-unauthenticated \
#   --memory=256Mi \
#   --platform=managed \
#   --region=us-central1 \
#   --project=$proj

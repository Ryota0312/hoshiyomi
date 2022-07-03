for i in `seq 1 31`
do
  day=$(printf "%02d\n" "${i}")
  data="{\"date\": \"2022-07-${day}T00:00:00.000Z\", \"longitude\": \"133.92\", \"latitude\": \"34.54\"}"
  grpcurl -plaintext -import-path ./proto -proto moon.proto -d "$data" [::]:50051 moon.MoonApi/MoonInfo
done

#!/bin/bash

HOST="http://localhost:8080/api/v1"
POST_NUM=100

user_id=$(curl -X GET -k -s "$HOST/users?id_start=0&id_end=99999&page=1&page_size=10" \
        -H 'Content-Type: application/json' | jq -r '.posts[0].id')

echo "Adding $POST_NUM posts using user id:$user_id"
for i in $(seq 1 $POST_NUM);
do
    result=$(curl -X POST -k -s $HOST/posts \
            -H 'Content-Type: application/json' \
            -d '
            {
                "user_id": '$user_id',
                "title": "title-'$i'",
                "content": "hello world '$i'!!!"
            }
            '
            )
    echo "$result"
done

first_id=$(curl -X GET -k -s "$HOST/posts?id_start=0&id_end=$POST_NUM&page=1&page_size=10" \
        -H 'Content-Type: application/json' | jq -r '.posts[0].id')
echo "first post id: $first_id"

echo "GET post $first_id"
result=$(curl -X GET -k -s "$HOST/posts/$first_id" \
            -H 'Content-Type: application/json')
echo "$result"

echo "PUT post $first_id"
result=$(curl -X PUT -k -s $HOST/posts \
        -H 'Content-Type: application/json' \
        -d '
        {
            "id": '$first_id',
            "title": "title-'$first_id' has changed",
            "content": "content of '$first_id' has changed"
        }
        '
        )
echo "$result"

echo "Read post $first_id"
result=$(curl -X POST -k -s "$HOST/posts/read/$first_id" \
        -H 'Content-Type: application/json')
echo "$result"

echo "List posts ..."
result=$(curl -X GET -k -s "$HOST/posts?id_start=0&id_end=$POST_NUM&page=1&page_size=10" \
        -H 'Content-Type: application/json' | jq)
echo "$result"

echo "DELETE first posts ..."
result=$(curl -X DELETE -k -s "$HOST/posts/$first_id" \
        -H 'Content-Type: application/json')
echo "$result"
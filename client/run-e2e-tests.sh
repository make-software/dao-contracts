#!/bin/sh
mkdir -p .casper-node
docker-compose up -d
echo "Waiting 60s for casper-node to start."
sleep 60
cp .env.example .env
npm install
npm run e2e:generic-client
docker-compose down

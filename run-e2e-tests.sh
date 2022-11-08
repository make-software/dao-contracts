#!/bin/sh
docker-compose up -d
sleep 5
cp .env.example client/.env
cp .env.example .env
cd client
npm install
npm run e2e:generic-client
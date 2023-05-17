VERSION 0.7

github-readme-stats:
    FROM node:18-alpine

    # Create app directory
    WORKDIR /app

    # Install (https://github.com/anuraghazra/github-readme-stats#on-other-platforms)
    COPY github-readme-stats /app
    SAVE IMAGE --push docker.io/crusaders/github-readme-stats-docker-ex
    RUN npm install express
    EXPOSE 9000
    CMD [ "node", "express.js" ]
    SAVE IMAGE --push docker.io/crusaders/github-readme-stats-docker

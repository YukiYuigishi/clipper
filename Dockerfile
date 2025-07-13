# Use a Node.js base image since gemini-cli is a Node.js application
FROM node:24-bookworm

# Install Go, git, and other basic development tools
#RUN apk update && apk add --no-cache go git bash
RUN apt-get update && apt-get install golang git bash -y


# Set the working directory for your projects
WORKDIR /workspace

# Install the Gemini CLI globally within the container
RUN npm install -g @google/gemini-cli

# Keep the container running so you can `docker exec` into it for development
#CMD ["tail", "-f", "/dev/null"]

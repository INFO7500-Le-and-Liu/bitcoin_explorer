# use node 16.7
FROM node:16.7

# install angular cli
RUN npm install -g @angular/cli@12.2.0

# set work dir
WORKDIR /app

COPY package*.json ./

RUN npm install

COPY . /app

# EXPOSE 4200 port
EXPOSE 4200

# launch the countainer
# use: ng serve
CMD ["ng", "serve", "--host", "0.0.0.0"]

#
docker pull nearprotocol/nearup
docker exec nearup nearup stop
docker rm nearup
docker run -v $HOME/.near:/root/.near -p 3030:3030 -d --name nearup nearprotocol/nearup run $1
docker logs -f nearup

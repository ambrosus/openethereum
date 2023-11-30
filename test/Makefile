run: copy-spec start

copy-spec:
	cp ./amb/apollo0/chain.json ./amb/apollo1/chain.json && \
    cp ./amb/apollo0/chain.json ./amb/apollo2/chain.json


start:
	docker-compose up -d

stop:
	docker-compose down

clear:
	docker-compose down --volumes && \
	rm -rf ./amb/apollo0/chains && \
    rm -rf ./amb/apollo1/chains && \
    rm -rf ./amb/apollo2/chains && \
    docker-compose rm -f  && \
    docker volume prune -f


integration-tests: stop clear copy-spec start

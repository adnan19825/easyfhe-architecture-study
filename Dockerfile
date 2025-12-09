FROM ubuntu:22.04
RUN apt-get update && apt-get install -y python3-pip git
# Installiere Dependencies (Simuliert für PoC)
RUN pip3 install numpy
# Kopiere Code
COPY poc_demo/ /app/
WORKDIR /app
CMD ["python3", "cifar_fhe.py"]

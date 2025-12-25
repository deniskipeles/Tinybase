FROM alpine:latest

# Install dependencies
# ca-certificates: for HTTPS/SSL
# libstdc++: required for many Rust binaries
# gcompat: provides glibc compatibility layer (in case binary wasn't compiled for musl)
RUN apk add --no-cache \
    ca-certificates \
    libstdc++ \
    tar

WORKDIR /app

# 1️⃣ Copy everything EXCEPT the raw binary (assumes .dockerignore handles exclusion if needed)
COPY . .

# 2️⃣ Extract the binary
RUN tar -xzf apexkit-api.tar.gz \
    && chmod +x apexkit-api \
    && rm apexkit-api.tar.gz

EXPOSE 8080
ENV HOST=0.0.0.0
ENV PORT=8080

CMD ["./apexkit-api", "--port", "8080"]

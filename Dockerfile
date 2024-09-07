FROM alpine:latest AS builder
ARG TARGETARCH
WORKDIR /builder
COPY . .
RUN ./sesamo-backend-* ./sesamo-backend

FROM scratch
COPY --from=builder --chmod=755 /builder/sesamo-backend ./sesamo-backend
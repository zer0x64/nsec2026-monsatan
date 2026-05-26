# Generate the CA private key
openssl ecparam -name prime256v1 -genkey -noout -out ca-key.pem

# Generate the CA self-signed certificate
openssl req -new -x509 -key ca-key.pem -out ca-cert.pem -days 3650 -subj "/CN=dartreg"

# Generate the server private key
openssl ecparam -name prime256v1 -genkey -noout -out key.pem

# Generate the server certificate signing request
openssl req -new -key key.pem -out req.csr -config cert.cnf -extensions req_ext

# Sign the server certificate with the CA
openssl x509 -req -in req.csr -out cert.pem -CA ca-cert.pem -CAkey ca-key.pem -CAcreateserial -days 3650 -extfile cert.cnf -extensions req_ext

cat cert.pem ca-cert.pem > chain.pem

# Clean up
rm req.csr

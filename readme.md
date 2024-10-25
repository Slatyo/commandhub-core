# Installation

### Clone the repository

```bash
git clone https://github.com/yourusername/emergency-deployment-server.git
cd emergency-deployment-server
```

### Generate RSA Private Key and Self-Signed Certificate

Using a 2048-bit RSA private key and a corresponding self-signed X.509 certificate generated with OpenSSL for TLS encryption.  
 

```bash
openssl req -x509 -newkey rsa:2048 -nodes -keyout key.pem -out cert.pem -days 365
```

### Build the server

```bash
cargo build
```

# Configuration

Create the `.env` file  

```bash
cp .env.example .env
```



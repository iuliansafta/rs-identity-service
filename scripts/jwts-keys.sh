#!/bin/bash
# Script to generate JWT RSA keys for local development
# This will be used as a devbox init hook

set -e

KEYS_DIR="./keys"
PRIVATE_KEY_FILE="${KEYS_DIR}/jwt_private_key.pem"
PUBLIC_KEY_FILE="${KEYS_DIR}/jwt_public_key.pem"

green='\033[0;32m'
yellow='\033[1;33m'
red='\033[0;31m'
reset='\033[0m'

echo -e "${yellow}Checking for JWT keys...${reset}"

if [ ! -d "$KEYS_DIR" ]; then
    echo -e "${yellow}Creating keys directory...${reset}"
    mkdir -p "$KEYS_DIR"
    echo -e "${green}✓ Keys directory created.${reset}"
fi

if [ ! -f "$PRIVATE_KEY_FILE" ]; then
    echo -e "${yellow}Generating RSA private key...${reset}"
    openssl genrsa -out "$PRIVATE_KEY_FILE" 2048
    echo -e "${green}✓ RSA private key generated.${reset}"
else
    echo -e "${green}✓ RSA private key already exists.${reset}"
fi

if [ ! -f "$PUBLIC_KEY_FILE" ]; then
    echo -e "${yellow}Extracting RSA public key...${reset}"
    openssl rsa -in "$PRIVATE_KEY_FILE" -pubout -out "$PUBLIC_KEY_FILE"
    echo -e "${green}✓ RSA public key extracted.${reset}"
else
    echo -e "${green}✓ RSA public key already exists.${reset}"
fi

chmod 600 "$PRIVATE_KEY_FILE"
chmod 644 "$PUBLIC_KEY_FILE"

echo -e "${green}JWT keys are ready for local development!${reset}"
echo -e "Private key: ${PRIVATE_KEY_FILE}"
echo -e "Public key: ${PUBLIC_KEY_FILE}"
echo

echo -e "${yellow}How to use these keys:${reset}"
echo -e "1. In your local config, set the paths to these key files:"
echo -e "   JWT_PRIVATE_KEY_PATH=\"${PRIVATE_KEY_FILE}\""
echo -e "   JWT_PUBLIC_KEY_PATH=\"${PUBLIC_KEY_FILE}\""
echo
echo -e "2. Use the LoadRSAPrivateKey and LoadRSAPublicKey functions to load these keys"
echo -e "   - For signing tokens: LoadRSAPrivateKey(\"${PRIVATE_KEY_FILE}\")"
echo -e "   - For verifying tokens: LoadRSAPublicKey(\"${PUBLIC_KEY_FILE}\")"
echo
echo -e "${red}IMPORTANT: Never commit these keys to version control!${reset}"
echo -e "Make sure ${KEYS_DIR}/ is in your .gitignore file."

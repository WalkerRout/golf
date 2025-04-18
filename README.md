# golf

Literally just generating keys with:
`openssl req -x509 -nodes -days 365 -newkey rsa:2048 -keyout ./nginx/ssl/nginx.key -out ./nginx/ssl/nginx.crt`
instead of certbot cause im lazy...
My personal site

Hosted at [heffree.dev](https://heffree.dev/)


nginx config:
```nginx
server {
        listen 443 ssl;
        server_name heffree.dev www.heffree.dev;

        ssl_certificate /etc/letsencrypt/live/heffree.dev/fullchain.pem;
        ssl_certificate_key /etc/letsencrypt/live/heffree.dev/privkey.pem;
        include /etc/letsencrypt/options-ssl-nginx.conf;
        ssl_dhparam /etc/letsencrypt/ssl-dhparams.pem;

        root /var/lib/heffree-dev;
        index index.html;

        location / {
                try_files $uri $uri/ =404;
        }
}
```

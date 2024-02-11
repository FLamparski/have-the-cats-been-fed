# Have the cats been fed?

Simple app to tell us if the cats have been fed.

## Architecture

Database: sqlite. Tables: `feed_log(ts: datetime, device_id: text)`.

Backend: anything but the JVM.

| endpoint                  | description                                      |
| ------------------------- | ------------------------------------------------ |
| `POST /log`               | Adds a new entry for when the cats have been fed |
| `GET /log?page=0&size=10` | Gets recent log entries                          |
| `WS /ws`                  | Reserved for websockets                          |

Frontend: NuxtJS (Vue 3) PWA.

## Deployment

Everything deploys to Olaf. Available over the internet at:

* https://have-the-cats-been-fed.internal.filipwieland.com

This requires an entry in CloudFlare Tunnels.

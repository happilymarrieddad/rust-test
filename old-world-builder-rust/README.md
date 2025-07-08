Basic Rust API
===================

## Example requests
```bash
☁  old-world-builder-rust [master] ⚡  curl 'localhost:8080/v1/users?limit=50&offset=0&email='
{"users":[{"id":161,"first_name":"Nick","last_name":"Kotenberg","email":"nick@mail.com","password":"$2b$12$0V.ZIwiMpSwvpndZQrclieMXprlT3oMj/Ckup.souI7yHf./mv.5m","created_at":"2025-07-08T01:48:43.434354Z","updated_at":"2025-07-08T01:48:43.434354Z","active":true,"deleted_at":null}],"total":1}%
☁  old-world-builder-rust [master] ⚡  curl -X POST -d '{"first_name":"nick2","last_name":"kotenberg","email":"nick2@mail.com","password":"1234","password_confirm":"1234"}' 'localhost:8080/v1/users'
wrong number of parameters: 0 expected 1%
☁  old-world-builder-rust [master] ⚡  curl -X POST -d '{"first_name":"nick2","last_name":"kotenberg","email":"nick2@mail.com","password":"1234","password_confirm":"1234"}' -H "Content-Type:application/json" 'localhost:8080/v1/users'
wrong number of parameters: 0 expected 1%
☁  old-world-builder-rust [master] ⚡  curl -X POST -d '{"first_name":"nick2","last_name":"kotenberg","email":"nick2@mail.com","password":"1234","password_confirm":"1234"}' -H "Content-Type:application/json" 'localhost:8080/v1/users'
{"id":163,"first_name":"nick2","last_name":"kotenberg","email":"nick2@mail.com","password":"$2b$12$IcYFx3CM5YyHVznDa4lL1ernxhGjAaVldb0FwF6k40zmslQNdU6vq","created_at":"2025-07-08T02:29:40.308900Z","updated_at":"2025-07-08T02:29:40.308900Z","active":true,"deleted_at":null}%
☁  old-world-builder-rust [master] ⚡  curl -X POST -d '{"first_name":"nick2","last_name":"kotenberg","email":"nick3@mail.com","password":"1234","password_confirm":"12345"}' -H "Content-Type:application/json" 'localhost:8080/v1/users'
{"message":"password must match password confirm","status":400}%
☁  old-world-builder-rust [master] ⚡  curl -X POST -d '{"first_name":"nick2","last_name":"kotenberg","email":"nick3@mail.com","password":"1234","password_confirm":"12345"}' -H "Content-Type:application/json" 'localhost:8080/v1/users'
☁  old-world-builder-rust [master] ⚡  curl -X POST -d '{"first_name":"nick2","last_name":"kotenberg","email":"nick2@mail.com","password":"1234","password_confirm":"1234"}' -H "Content-Type:application/json" 'localhost:8080/v1/users'
{"message":"error returned from database: duplicate key value violates unique constraint \"users_email_key\"","status":400}%
☁  old-world-builder-rust [master] ⚡  curl -X POST -d '{"first_name":"nick2","last_name":"kotenberg","email":"nick3@mail.com","password":"1234","password_confirm":"1234"}' -H "Content-Type:application/json" 'localhost:8080/v1/users'
{"id":165,"first_name":"nick2","last_name":"kotenberg","email":"nick3@mail.com","password":"$2b$12$vkOMpZx4X5.SRTibXvPF6.ntjhTXEJnLHBlfZSAKUeTusgQcA122y","created_at":"2025-07-08T02:30:32.531549Z","updated_at":"2025-07-08T02:30:32.531549Z","active":true,"deleted_at":null}%
☁  old-world-builder-rust [master] ⚡  curl 'localhost:8080/v1/users?limit=50&offset=0&email='
{"users":[{"id":161,"first_name":"Nick","last_name":"Kotenberg","email":"nick@mail.com","password":"$2b$12$0V.ZIwiMpSwvpndZQrclieMXprlT3oMj/Ckup.souI7yHf./mv.5m","created_at":"2025-07-08T01:48:43.434354Z","updated_at":"2025-07-08T01:48:43.434354Z","active":true,"deleted_at":null},{"id":163,"first_name":"nick2","last_name":"kotenberg","email":"nick2@mail.com","password":"$2b$12$IcYFx3CM5YyHVznDa4lL1ernxhGjAaVldb0FwF6k40zmslQNdU6vq","created_at":"2025-07-08T02:29:40.308900Z","updated_at":"2025-07-08T02:29:40.308900Z","active":true,"deleted_at":null},{"id":165,"first_name":"nick2","last_name":"kotenberg","email":"nick3@mail.com","password":"$2b$12$vkOMpZx4X5.SRTibXvPF6.ntjhTXEJnLHBlfZSAKUeTusgQcA122y","created_at":"2025-07-08T02:30:32.531549Z","updated_at":"2025-07-08T02:30:32.531549Z","active":true,"deleted_at":null}],"total":3}%
☁  old-world-builder-rust [master] ⚡  curl 'localhost:8080/v1/users?limit=50&offset=1&email='
{"users":[{"id":163,"first_name":"nick2","last_name":"kotenberg","email":"nick2@mail.com","password":"$2b$12$IcYFx3CM5YyHVznDa4lL1ernxhGjAaVldb0FwF6k40zmslQNdU6vq","created_at":"2025-07-08T02:29:40.308900Z","updated_at":"2025-07-08T02:29:40.308900Z","active":true,"deleted_at":null},{"id":165,"first_name":"nick2","last_name":"kotenberg","email":"nick3@mail.com","password":"$2b$12$vkOMpZx4X5.SRTibXvPF6.ntjhTXEJnLHBlfZSAKUeTusgQcA122y","created_at":"2025-07-08T02:30:32.531549Z","updated_at":"2025-07-08T02:30:32.531549Z","active":true,"deleted_at":null}],"total":3}%
☁  old-world-builder-rust [master] ⚡  curl 'localhost:8080/v1/users?limit=50&offset=2&email='
{"users":[{"id":165,"first_name":"nick2","last_name":"kotenberg","email":"nick3@mail.com","password":"$2b$12$vkOMpZx4X5.SRTibXvPF6.ntjhTXEJnLHBlfZSAKUeTusgQcA122y","created_at":"2025-07-08T02:30:32.531549Z","updated_at":"2025-07-08T02:30:32.531549Z","active":true,"deleted_at":null}],"total":3}%
☁  old-world-builder-rust [master] ⚡  curl 'localhost:8080/v1/users?limit=1&offset=0&email='
{"users":[{"id":161,"first_name":"Nick","last_name":"Kotenberg","email":"nick@mail.com","password":"$2b$12$0V.ZIwiMpSwvpndZQrclieMXprlT3oMj/Ckup.souI7yHf./mv.5m","created_at":"2025-07-08T01:48:43.434354Z","updated_at":"2025-07-08T01:48:43.434354Z","active":true,"deleted_at":null}],"total":3}%
☁  old-world-builder-rust [master] ⚡  curl -X PUT -d '{"first_name":"nick3","last_name":"kotenberg"}' -H "Content-Type:application/json" 'localhost:8080/v1/users/165'
{"id":165,"first_name":"nick3","last_name":"kotenberg","email":"nick3@mail.com","password":"$2b$12$vkOMpZx4X5.SRTibXvPF6.ntjhTXEJnLHBlfZSAKUeTusgQcA122y","created_at":"2025-07-08T02:30:32.531549Z","updated_at":"2025-07-08T02:30:32.531549Z","active":true,"deleted_at":null}%
☁  old-world-builder-rust [master] ⚡  curl 'localhost:8080/v1/users?limit=3&offset=0&email='
{"users":[{"id":161,"first_name":"Nick","last_name":"Kotenberg","email":"nick@mail.com","password":"$2b$12$0V.ZIwiMpSwvpndZQrclieMXprlT3oMj/Ckup.souI7yHf./mv.5m","created_at":"2025-07-08T01:48:43.434354Z","updated_at":"2025-07-08T01:48:43.434354Z","active":true,"deleted_at":null},{"id":163,"first_name":"nick2","last_name":"kotenberg","email":"nick2@mail.com","password":"$2b$12$IcYFx3CM5YyHVznDa4lL1ernxhGjAaVldb0FwF6k40zmslQNdU6vq","created_at":"2025-07-08T02:29:40.308900Z","updated_at":"2025-07-08T02:29:40.308900Z","active":true,"deleted_at":null},{"id":165,"first_name":"nick3","last_name":"kotenberg","email":"nick3@mail.com","password":"$2b$12$vkOMpZx4X5.SRTibXvPF6.ntjhTXEJnLHBlfZSAKUeTusgQcA122y","created_at":"2025-07-08T02:30:32.531549Z","updated_at":"2025-07-08T02:30:32.531549Z","active":true,"deleted_at":null}],"total":3}%
☁  old-world-builder-rust [master] ⚡

☁  old-world-builder-rust [master] ⚡  curl -X POST -d '{"email":"nick@mail.com","password":"12345"}' -H "Content-Type:application/json" 'localhost:8080/login'
{"error":"unauthorized"}%
☁  old-world-builder-rust [master] ⚡  curl -X POST -d '{"email":"nick@mail.com","password":"1234"}' -H "Content-Type:application/json" 'localhost:8080/login'
{"status":"success","token":"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOjE2MSwicm9sZSI6InVzZXIiLCJleHAiOjE3NTE5NTg2OTN9.e14NGtpEWmx-U1w0d-gpW_66RH0sDLUUBkUDgjTRRJg"}%
☁  old-world-builder-rust [master] ⚡

☁  old-world-builder-rust [master] ⚡  curl -X POST -d '{"first_name":"nick4","last_name":"kotenberg","email":"nick4@mail.com","password":"1234","password_confirm":"1234"}' -H "Content-Type:application/json" -H "Authorization:Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOjE2MSwicm9sZSI6InVzZXIiLCJleHAiOjE3NTE5NTk0OTJ9.nd2ePJvtK5Dr0dqRmZmbEOzJQDJ7t_KA7SkQE1Ryhog" 'localhost:8080/v1/users'
{"id":166,"first_name":"nick4","last_name":"kotenberg","email":"nick4@mail.com","password":"$2b$12$I6TNO6McDjaHZ1moydu7x.4LEnEQZTsg1Mv1z3JqKky0Wybj1J5Si","created_at":"2025-07-08T03:27:09.982412Z","updated_at":"2025-07-08T03:27:09.982412Z","active":true,"deleted_at":null}%
☁  old-world-builder-rust [master] ⚡
```
// app
pub const APP_NAME: &str = "tauri-ws-poc";
pub const API_PATH: &str = "/api";
pub const HTTP_SERVER_KEEP_ALIVE: u64 = 30;
// env variables
pub const DEFAULT_HTTP_SERVER_URI: &str = "0.0.0.0:8443";
pub const DEFAULT_HTTP_SERVER_API_KEY: &str = "Uffpwzm5Ahx5zWVWi6H0LZnQnmYA4uelif2U54ATqDO0rORRMQRvnA1zuAnQkKU3bC6T4RI9EghIcwBaXKkenkT0t9jVdoaAMXQsKjFFGDn7oSvfTcSaU5YYKtY1ydwn";
pub const DEFAULT_HTTP_SERVER_ENABLE_HTTPS: bool = true;
// debug
pub const DEFAULT_LOG_ACTIXWEB_MIDDLEWARE_FORMAT: &str = r#""%r" %s %b "%{User-Agent}i" %D"#;
// spawn thread
pub const DEFAULT_SPAWN_THREAD_ENABLED: bool = false;
pub const DEFAULT_SPAWN_THREAD_DURATION_SECONDS: u64 = 30;

// bellow certificates are just for development environment,can be public exposed, no problemo

// raw certificates this is prevent expose and add tauri external files
pub const CERT_KEY_PEM: &str = r#"-----BEGIN PRIVATE KEY-----
MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQDDHr4/e9DV/0C/
zr0SGdfBhXzvKZ2tuHNRSb/cqNx9PD+73VBt0Pk7xY78xilZ/+76E69XQMmuo7gU
vY7bcUhIZBIULULO2eZ02tj7kJsSyZsnBv6nRAJk4OSKSU7WikKXLtqSp2OiK+CE
p2ocPVV2koKHlTH28mEpe95qBFuoXZqROY2y6EFiGBnZJJdRzYiczQJwM1egkZND
zWi++lpOI47kJeHpOqxYQDr2s7bz3w/OjG1OaP/HOJejLohsjXluNTCDF084wx99
3ut2p7CjFylFO/mk0LGS+LmoMR2bV5AppLjyq5JskTT72lejhgwXyQ2zkcS1VBP6
LdNzj4mlAgMBAAECggEAWCSbseb3dKyN7ffxHmuxNTldslac5dzEr0x/ESnxDNv6
OJ4mFq3H4TICQXstM/xzUuIwm9xnH27TGf91uPkIO7eFGTtHClWSD2jLi30MKKUN
Y0h8IkiNQk/7veKW7EDiCxS95XEbh6eFkbdlLmOgp2NQ9vm1bAabUNelpkPYugnf
VC3TviIbUacPA24di7g2Brhqx44gr0synWh/NDnJtJjWN00JwgvLotLLJ1KXHfn8
DDnuLyTiNhsKeHanH9AJmxuFLB4aoko7P/Xb5CVvB1k0CuRkzRkJ5GpBT9srAL+N
htBYsBSk5Hohvc0dIU2Z+rjYvc2tEB+kPmxO9z+LkwKBgQD9/cwCjn/uwI3JWbM5
jmkwzz2hH6LowAQDGfDZ7Afgc5Aw6w54HFGtr/KkulIm4i2q1eduDmGX4jP4PiQ5
N1ksaZu2yWXXX38OAR6s5xuhmf+Utc7hlptPHza/y05FejMfA8g0tgJ52INyw2c8
LPZlMeTF6QV3aPJre1t0AjON4wKBgQDEqcMJmenY5DJzTnyK8yW5KjjmrIfV/DPy
LCkjs9kgtmMtwYPE072nehgJMoEK+AycooC8iZw/paRG0ZTJZSjOKQhhSaJCmeN4
y9Ko12sjT3Ww+3VhZ3OjmFZ3I+VSpm6Dtb2ZWomajQynf1XDnlk3ZECgeDqbGPf+
ExnNfA0g1wKBgHjiyq6M1Cb2ToV+w+ao+TfZ6yM8w3SzrzUqq4+M31LcrcwalBBm
qhlYnMv6Qg9vpvygQj4Haju31bRZHzR8O10ABrTAtlADi51Guyrf9C699P76aTTF
UbYezRgL030N5DbmvesYlaUYf7eneQMTA1K2pIPtnERxFrBBTg2w3oixAoGBAMEq
CVL95W3Ovir5lWSZbJ0R9q/EGOu6e39LluDlfsgdvQ6tO3Lxkx/T0ZX/EDao6yJS
juYgt+LgHJoesuiZvtm3/e4o+AI4yu/UK0cBx8739h5u7p1Mko3GTG9bR5zFslkW
AnOFNREjp6FBdgjdGL5Kbxvu24mJndPmCPsG9JIjAoGAJ6EvO87tPByt9GZVVGpO
YS+tETzHKaMvGf4W/8yGWD0jp8G+n5yugKQoApoKHUSs8G8oOhdcXkOpNl6lzD9T
QaWdMzXzt6XXnGRIQwSsQ6hgdUXzYZVtYiHcIt4q8+ZS69tb030MPp8F8Bz214pD
FJtze556382I7VWjjLflaTw=
-----END PRIVATE KEY-----"#;

pub const CERT_CERT_FILE: &str = r#"-----BEGIN CERTIFICATE-----
MIIEEjCCAvqgAwIBAgIURqV92LXAvzmu5UCctP05Df/4uAgwDQYJKoZIhvcNAQEL
BQAwgY4xCzAJBgNVBAYTAlBUMRAwDgYDVQQIDAdDb2ltYnJhMRgwFgYDVQQHDA9G
aWd1ZWlyYSBkYSBGb3oxEDAOBgNVBAoMB0t1YXJ0em8xDDAKBgNVBAsMA0RldjES
MBAGA1UEAwwJMTI3LjAuMC4xMR8wHQYJKoZIhvcNAQkBFhBtYWlsQGt1YXJ0em8u
b3JnMCAXDTIzMTEzMDIzNDM1MVoYDzIxMjMxMTA2MjM0MzUxWjCBjjELMAkGA1UE
BhMCUFQxEDAOBgNVBAgMB0NvaW1icmExGDAWBgNVBAcMD0ZpZ3VlaXJhIGRhIEZv
ejEQMA4GA1UECgwHS3VhcnR6bzEMMAoGA1UECwwDRGV2MRIwEAYDVQQDDAkxMjcu
MC4wLjExHzAdBgkqhkiG9w0BCQEWEG1haWxAa3VhcnR6by5vcmcwggEiMA0GCSqG
SIb3DQEBAQUAA4IBDwAwggEKAoIBAQDDHr4/e9DV/0C/zr0SGdfBhXzvKZ2tuHNR
Sb/cqNx9PD+73VBt0Pk7xY78xilZ/+76E69XQMmuo7gUvY7bcUhIZBIULULO2eZ0
2tj7kJsSyZsnBv6nRAJk4OSKSU7WikKXLtqSp2OiK+CEp2ocPVV2koKHlTH28mEp
e95qBFuoXZqROY2y6EFiGBnZJJdRzYiczQJwM1egkZNDzWi++lpOI47kJeHpOqxY
QDr2s7bz3w/OjG1OaP/HOJejLohsjXluNTCDF084wx993ut2p7CjFylFO/mk0LGS
+LmoMR2bV5AppLjyq5JskTT72lejhgwXyQ2zkcS1VBP6LdNzj4mlAgMBAAGjZDBi
MB0GA1UdDgQWBBT75K03QZ6sxWYNGNSYFuACe5nWTzAfBgNVHSMEGDAWgBT75K03
QZ6sxWYNGNSYFuACe5nWTzAPBgNVHRMBAf8EBTADAQH/MA8GA1UdEQQIMAaHBH8A
AAEwDQYJKoZIhvcNAQELBQADggEBAEvvQZizHl4FzhFL3U8ymI2a0joDlvYq07Tz
RnfoVxfEao0zSKf+rtvmfHSrPC8ZnLX1Q0r/WEy8d1duDBudTF5yPZHSlsc1o48X
eeidAR9uPo9flyywt5MXXWBuQuXiLm6eP69Ybfqs5sma6nbsZuFqXv8fZnYjYti1
Glsn0U2DbiJLfcjGI6JrTtVOGMCk0/AtIvXHqSlx8jdmsWZcmcTrpq4OGxWC7PHZ
w9/jNq5fzcaraJhoQtKuTVOz1ys57XzoINTmbpAEmhqpjPM9gspT+gTghdNP6b6F
k9IlXMlH/ubuGVYdHtbr2Edmh3EtozrcxaxrSgmAg5NsYN7rhdQ=
-----END CERTIFICATE-----"#;
<?php 

#[\Since('8.4')]
function sodium_crypto_aead_aegis256_encrypt(#[\SensitiveParameter] string $message, string $additional_data, string $nonce, #[\SensitiveParameter] string $key): string
{
}
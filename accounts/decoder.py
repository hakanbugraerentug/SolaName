import solana
import base58
from solders.keypair import Keypair

# Generate keypair object
key_array = bytes([
  45, 29, 54, 192, 238, 38, 144, 13, 41, 149, 73, 135, 77, 49, 156, 121, 9, 241,
  181, 179, 166, 106, 182, 70, 146, 180, 227, 71, 29, 179, 213, 249, 147, 205,
  1, 2, 80, 210, 33, 203, 95, 187, 131, 7, 116, 185, 210, 65, 215, 91, 79, 191,
  109, 65, 35, 188, 168, 193, 131, 103, 35, 43, 99, 105])
generated_keypair = Keypair.from_bytes(key_array)


print("Keypair ::", generated_keypair) #Public Key

print("Public Key::", generated_keypair.pubkey()) #Public Key
print("Secret Key::", base58.b58encode(generated_keypair.secret()).decode('utf-8')) #Raw 64-byte secret key
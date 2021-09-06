# tfhe_vaxxbox

A confidential counter for full vaccination against covid-19 under homomorphic encryption in the Concrete (TFHE) library for the Rust programming language. Participants register their full vaccination status (yes, no, prefer not to tell) which is added to running sums that are communicated in a ring-type directed graph (think call chain). 

1. The administrator initiates the sum by sending encrypted zeros to the first person on the chain, 
2. each participant votes and then pass the running sums to the next person on the chain, 
3. which stops at full circle with the admininstrator who has the key to decode the result. 

create_keys 
- run this to create a secret key for encryption (no need for key switching and bootstrap keys)

test_basic 
- test some basic (levelled) operations like addition, multiplication and negation (sign flip)

test_basic 
- test the operations of a hypothetical chain, i.e. init a zero, vote of each type, decrypt results

init_zero
- initialize the summation as an encrypted vector of zeroes using the key in the keys directory - save to disk as sum.enc

vote_yes
- read results vector from sum.enc, add one to the affirm position, save to disk as sum.enc

vote_no
- read results vector from sum.enc, add one to the vote_no position, save to disk as sum.enc

vote_nul
- read results vector from sum.enc, do not add to any vote position, save to disk as sum.enc

decrypt
- decrypt the results using the secret key from the keys directory# tfhe_vaxxbox

The application gets it name from the similarity of the algorithm to a real life situation how a physical voting box could be passed in a chain (or tree) between participants after each has inserted their secret vote. 
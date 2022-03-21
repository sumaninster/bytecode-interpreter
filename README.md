# Composable Finance ByteCode Interpreter
Sample byte codes are in code folder. 
If you want to add more byte code just add it to code folder with .bc extension.

Run the below command from command line to execute all test cases:

$ cargo test -- --nocapture
## Syntax

LOAD_VAL 1 - pushes value 1 into stack

WRITE_VAR x - pops from stack and writes to "x" variable

READ_VAR x - reads value of "x" and pushes to stack

ADD - pops two values from stack and adds

ADD - pops two values from stack and adds

SUBTRACT - pops two values from stack and subtracts

MULTIPLY - pops two values from stack and multiplies 

DIVIDE - pops two values from stack and divides

LESS_THAN - pops two values from stack and compares for less than 

LESS_THAN_EQUAL - pops two values from stack and compares for less than equal

GREATER_THAN - pops two values from stack and compares for greater than

GREATER_THAN_EQUAL - pops two values from stack and compares for greater than equal

RETURN - exists function without any return value

RETURN_VALUE - exists function with return value

PRINT x - prints value of "x" to terminal (no new line)

PRINT_LN x - prints value "x" to terminal with new line

SLEEP 5 - sleeps for five seconds

LOOP - start of loop condition block

LOOP_START - start of loop code block

LOOP_END - end of loop

FUNC add - creates a function name add

FUNC_END - end of the function block

FUNC_CALL add - calls function add and passes all variables from current execution block

FUNC_CALL send sch - calls function send and passes all variables from current execution block and moves additional channel specific parameter "sch"

SPAWN sch - starts a new thread and moves sch to thread block

SPAWN_END - end of thread block

CHANNEL sch rch - creates new channel with sender: "sch" and receiver: "rch" end points

SEND_CHANNEL sch - pops value from stack and writes to channel "sch"

RECEIVE_CHANNEL rch - reads from channel "rch" and pushes to stack

#Question and Answer
### (3) Suppose we added the following bytecode instructions to our language:
SEND_CHANNEL:
Pops the channel and a value from the stack and send the value on the channel using a blocking send
RECV_CHANNEL:
Pops the channel from the stack, receives a value from the channel (this may block), and push the resulting value back onto the stack
SPAWN:
Pop two functions from the stack and spawn them as concurrent tasks
Describe in a few sentences how each bytecode instruction could be interpreted, and how your interpreter or language runtime could deal with the blocking nature of send and receive instructions.

Ans: I have implemented channels in this interpreter (code_thread.bc is example byte code to test channels) using rust channels. 
Rust channels uses feature call atomic counter in most situation and only in worst cases it uses mutex to handle blocking nature.

### (5) explain some ways hashing functions enable blockchain technology
Ans: Hash function are used to construct merkle trees and also to generate block header hash, which is th root of merkle tree in a block.

### (6) briefly explain Bitcoin's UTXO model of transaction validation (separate from POW)
Ans: When someone receives bitcoin, the transaction is recorded as UTXO (unspent transaction output). 
When someone transfers bitcoin to someone else, the balance from this transaction forms a return change transaction that also forms UTXO, this UTXO forms the input for the next transaction from this account. 
The UTXO's that are consumed for a transaction are called transaction inputs. The UTXO's that are created by a transaction are called transaction outputs. 

### (7) what is the structure of a Block in bitcoin and how does it relate to the 'blockchain' (merkle tree vs merkle list of merkle trees)
Ans: Each block in bitcoin consists of block size, block header, transaction counter and transactions. 
The block header consists of version, previous block hash, merkle root, timestamp, difficulty target and nonce. 
The blocks are tied into a chain with the root/header hash of previous block, that forms the blockchain.
Merkle tree is linked binary tree of hash of transactions. Transactions hashed, two hash of transaction is hashed again to form the root of those two transaction.
Merkle list is merkle tree stored in list (array).

### (8) what problem/s are POW/POS trying to solve? discuss/compare (byzantine fault tolerance, reaching a single consensus on a p2p network)
Ans: POW/POS is implements to ensure that nodes does not behave inappropriately. 
Byzantine fault tolerance assumes that certain number of nodes will be down or will provide faulty data. 
For example if 2/3 of nodes provides same outcome for a computational work and 1/3 provides faulty data, then the data received from 2/3 is accepted into the system.
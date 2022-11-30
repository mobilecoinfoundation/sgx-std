#include <stddef.h>

void ocall_stderr(const void * input, size_t len);

// Copyright (c) 2022 The MobileCoin Foundation
/*
 * A basic function that just adds 2 to its input
 */
void ecall_add_2(int input, int *sum) {
    *sum = input + 2;
}

/*
 * A thin wrapper that will take the provided message from the untrusted side
 * and pipe it back out through an ocall to the untrusted side again...
 * This shows the passing of messages via stderr through the enclave
 *
 * \param input: The input message. Since this will be used in rust, assume
 *  utf8.
 * \param len: The length of input, in bytes
 */
void ecall_round_trip_to_stderr(const void* input, size_t len) {
    ocall_stderr(input, len);
}


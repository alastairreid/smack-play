/****************************************************************
 * Singly linked list
 *
 * Copyright Alastair Reid 2020
 * SPDX-Licence-Identifier: BSD-3-Clause
 ****************************************************************/

struct node {
    int data;
    struct node *next;
};

struct node *cons(int x, struct node *xs);
int head(struct node *l);
struct node* tail(struct node *l);
// combined head/tail function
int pop(struct node **pl);
void list_dispose(struct node *l);

/****************************************************************
 * End
 ****************************************************************/

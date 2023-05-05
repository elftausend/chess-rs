#ifndef CHESSPP_CHESSRS_H
#define CHESSPP_CHESSRS_H


#ifdef __cplusplus
extern "C" {
#endif
void chess_create(void **);

void chess_run(void *);

void chess_move(char start_x, char start_y, char dest_x, char dest_y);

void free_chess(void *);

#ifdef __cplusplus
}
#endif

#endif //CHESSPP_CHESSRS_H

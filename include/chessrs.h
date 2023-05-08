#ifndef CHESSPP_CHESSRS_H
#define CHESSPP_CHESSRS_H


#ifdef __cplusplus
namespace chessrs {
    extern "C" {
#endif
    enum Team {white=0,black=1};

    struct Move {
        char start_x, start_y, dest_x, dest_y;
    };

    void chess_create(void **);
    void chess_run(void *);
    void chess_move(void *, char start_x, char start_y, char dest_x, char dest_y);
    void free_chess(void *);
    Team chess_get_current_team(void*);
    void chess_set_current_team(void*, Team);
    Move chess_get_latest_move(void*);




#ifdef __cplusplus
    }
}
#endif

#endif //CHESSPP_CHESSRS_H

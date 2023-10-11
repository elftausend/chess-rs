#ifndef CHESSPP_CHESSRS_H
#define CHESSPP_CHESSRS_H


#ifdef __cplusplus
namespace chessrs {
    extern "C" {
#endif
    enum Team {white=0,black=1};

    enum FigureType {
        Pawn=0,
        King,
        Queen,
        Knight,
        Rook,
        Bishop,
    };


    struct Move {
        size_t start_y, start_x, dest_y, dest_x;
    };



    void chess_create(void **);
    void chess_run(void *);
    void chess_move(void *, size_t start_y, size_t start_x, size_t dest_y, size_t dest_x);
    void chess_free(void *);
    Team chess_get_current_team(void*);
    void chess_set_current_team(void*, Team);
    Move chess_get_latest_move(void*);


    void chess_set_figure_at(void*, size_t y, size_t x, FigureType figure, Team team);
    FigureType chess_get_figure_at(void*, size_t y, size_t x);
    void chess_remove_figure_at(void*, size_t y, size_t x);




#ifdef __cplusplus
    }
}
#endif

#endif //CHESSPP_CHESSRS_H

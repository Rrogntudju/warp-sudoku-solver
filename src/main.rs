use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SudokuRequest {
    puzzle: String,
}

#[derive(Serialize)]
struct SolveResponse {
    status: String,
    data: String,
    message: String,
}

#[derive(Serialize)]
struct DisplayResponse {
    status: String,
    data: Vec<String>,
    message: String,
}

mod filters {
    use super::{handlers, SudokuRequest};
    use warp::Filter;

    pub fn sudoku() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path("api")
            .and(warp::post())
            .and(solve().or(display()))
    }

    pub fn solve() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path("solve")
            .and(warp::path::end())
            .and(json_body())
            .and_then(handlers::solve)
    }

    pub fn display() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path("display")
            .and(warp::path::end())
            .and(json_body())
            .and_then(handlers::display)
    }

    fn json_body() -> impl Filter<Extract = (SudokuRequest,), Error = warp::Rejection> + Clone {
        warp::body::content_length_limit(150).and(warp::body::json())
    }
}

mod sudoku;

mod handlers {
    use super::sudoku::Sudoku;
    use super::{DisplayResponse, SolveResponse, SudokuRequest};
    use std::convert::Infallible;

    pub async fn solve(req: SudokuRequest) -> Result<impl warp::Reply, Infallible> {
        let solve_result = Sudoku::new().solve(&req.puzzle);
        let sudoku_response = match solve_result {
            Ok(solution) => SolveResponse {
                status: "success".into(),
                data: solution,
                message: "".into(),
            },
            Err(e) => SolveResponse {
                status: "fail".into(),
                data: "".into(),
                message: format!("{}", e),
            },
        };

        Ok(warp::reply::json(&sudoku_response))
    }

    pub async fn display(req: SudokuRequest) -> Result<impl warp::Reply, Infallible> {
        let grid_result = Sudoku::display(&req.puzzle);
        let sudoku_response = match grid_result {
            Ok(grid) => DisplayResponse {
                status: "success".into(),
                data: grid,
                message: "".into(),
            },
            Err(e) => DisplayResponse {
                status: "fail".into(),
                data: Vec::new(),
                message: format!("{}", e),
            },
        };

        Ok(warp::reply::json(&sudoku_response))
    }
}

#[tokio::main]
async fn main() {
    let api = filters::sudoku();

    warp::serve(api).run(([127, 0, 0, 1], 7878)).await;
}

#[cfg(test)]
mod tests {
    use super::{filters, SudokuRequest};
    use warp::test::request;
    use warp::http::StatusCode;

    #[tokio::test]
    async fn solve_ok() {
        let api = filters::sudoku();
        let resp = request()
            .method("POST")
            .path("/api/solve")
            .json(&SudokuRequest {
                puzzle : "700000600060001070804020005000470000089000340000039000600050709010300020003000004".into()
            })
            .reply(&api)
            .await;
        
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(resp.body(),  
            r#"{"status":"success","data":"791543682562981473834726915356478291289615347147239568628154739415397826973862154","message":""}"#
        );
    }

    #[tokio::test]
    async fn display_ok() {
        let api = filters::sudoku();
        let resp = request()
            .method("POST")
            .path("/api/display")
            .json(&SudokuRequest {
                puzzle : "309800000000500000250009600480000097700000005930000061008300056000006000000007403".into()
            })
            .reply(&api)
            .await;

        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(resp.body(),  
            concat!(r#"{"status":"success","data":["3 0 9 |8 0 0 |0 0 0 ","0 0 0 |5 0 0 |0 0 0 ","2 5 0 |0 0 9 |6 0 0 ","------+------+------","#,
                r#""4 8 0 |0 0 0 |0 9 7 ","7 0 0 |0 0 0 |0 0 5 ","9 3 0 |0 0 0 |0 6 1 ","------+------+------","#,
                r#""0 0 8 |3 0 0 |0 5 6 ","0 0 0 |0 0 6 |0 0 0 ","0 0 0 |0 0 7 |4 0 3 "],"message":""}"#
            )
        );
    }

    #[tokio::test]
    async fn solve_err_puzzle() {
        let api = filters::sudoku();
        let resp = request()
            .method("POST")
            .path("/api/solve")
            .json(&SudokuRequest {
                puzzle : "X00000600060001070804020005000470000089000340000039000600050709010300020003000004".into()
            })
            .reply(&api)
            .await;

        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(resp.body(),  
            r#"{"status":"fail","data":"","message":"Invalid Grid.  Provide a string of 81 digits with 0 or . for empties."}"#
        );
    }

    #[tokio::test]
    async fn solve_err_json() {
        let api = filters::sudoku();
        let resp = request()
            .method("POST")
            .path("/api/solve")
            .body(r#"{"xuzzle": "700000600060001070804020005000470000089000340000039000600050709010300020003000004"}"#)
            .reply(&api)
            .await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }
}

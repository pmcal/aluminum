#[macro_export]
macro_rules! mat {
    ( $( [ $( $x:expr ),* $(,)? ] ),+ $(,)? ) => {{
        let data = vec![ $( $( $x as f32 ),* ),+ ];
        let rows = <[()]>::len(&[ $( mat!(@unit $($x),*) ),+ ]);
        let cols = mat!(@count_cols $( [ $( $x ),* ] ),+ );
        Mat::new(rows, cols, data)
    }};
    (@unit $($x:expr),*) => { () };
    (@count_cols [ $( $x:expr ),* ] $(, $rest:tt )* ) => {
        <[()]>::len(&[ $( mat!(@unit $x) ),* ])
    };
}

#[macro_export]
macro_rules! rvec {
    ( $( $x:expr ),* $(,)? ) => {
        Mat::new(1, <[()]>::len(&[ $( rvec!(@unit $x) ),* ]), vec![ $( $x as f32 ),* ])
    };
    (@unit $x:expr) => { () };
}

#[macro_export]
macro_rules! cvec {
    ( $( $x:expr ),* $(,)? ) => {
        Mat::new(<[()]>::len(&[ $( cvec!(@unit $x) ),* ]), 1, vec![ $( $x as f32 ),* ])
    };
    (@unit $x:expr) => { () };
}

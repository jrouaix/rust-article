use yew::html;

fn main() {
    let text = "lorem ipsum";
    html! {
        <>
            <div>{text}</div>
            <div>{"dolor sit"}</div>
            <span>{42}</span>
        </>
    }
    html! {
        <span></spa>
    }
}

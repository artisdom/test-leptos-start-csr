use leptos::*;

#[component]
pub fn Page6() -> impl IntoView {
    view! {
        <h1>"Iteration"</h1>
        <h2>"Static List"</h2>
        <p>"Use this pattern if the list itself is static."</p>
        <StaticList length = 5/>
        <h2>"Dynamic List"</h2>
        <p>"Use this pattern if the rows in your list will change."</p>
        <DynamicList initial_length = 5/>
    }
}

/// A list of counters, without the ability to add or remove any.
#[component]
pub fn StaticList(
    /// How many counters to include in this list.
    length: usize,
) -> impl IntoView {
    let counters = (1..=length).map(|idx| create_signal(idx));

    // when you have a list that doesn't change, you can
    // manipulate it using ordinary Rust iterators
    // and collect it into a Vec<_> to insert it into the DOM
    let counter_buttons = counters
        .map(|(count, set_count)| {
            view! {
                <li>
                    <button on:click = move |_| set_count.update(|n| *n += 1)>
                        {count}
                    </button>
                </li>
            }
        })
        .collect::<Vec<_>>();

    // Note that if `counter_buttons` were a reactive list
    // and its value changed, this would be very inefficient:
    // it would rerender every row every time the list changed.
    view! {
        <ul>{counter_buttons}</ul>
    }
}

/// A list of counters that allows you to add or remove counters.
#[component]
pub fn DynamicList(
    /// The number of counters to begin with.
    initial_length: usize,
) -> impl IntoView {
    // This dynamic list will use the <For/> component.
    // <For/> is a keyed list. This means that each row
    // has a defined key. If the key does not change, the row
    // will not be re-rendered. When the list changes, only
    // the minimum number of changes will be made to the DOM.

    // `next_counter_id` will let us generate unique IDs
    // we do this by simply incrementing the ID by one
    // each time we create a counter
    let mut next_counter_id = initial_length;

    // we generate an initial list as in <StaticList/>
    // but this time we include the ID along with the signal
    let initial_counters = (0..initial_length).map(|id| (id, create_signal(id + 1))).collect::<Vec<_>>();

    // now we store that initial list in a signal
    // this way, we'll be able to modify the list over time,
    // adding and removing counters, and it will change reactively
    let (counters, set_counters) = create_signal(initial_counters);

    let add_counter = move |_| {
        // create a signal for the new counter
        let sig = create_signal(next_counter_id + 1);

        set_counters.update(move |counters| {
            counters.push((next_counter_id, sig)) // since `.update()` gives us `&mut T` we can just use normal Vec methods like `push`
        });

        next_counter_id += 1; // increment the ID so it's always unique
    };

    view! {
        <div>
            <button on:click = add_counter>
                "Add Counter"
            </button>

            <ul>
                // The <For/> component is central here, This allows for efficient, key list rendering
                <For
                    each = counters
                    key = |counter| counter.0
                    children = move |(id, (count, set_count))| {
                        view! {
                            <li>
                                <button on:click = move |_| set_count.update(|n| *n += 1)>
                                    {count}
                                </button>
                                <button on:click = move |_| {
                                    set_counters.update(|counters| {
                                        counters.retain(|(counter_id, _)| counter_id != &id)
                                    });
                                }>
                                    "Remove"
                                </button>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}
use based::*;
use leptos::*;
use leptos_use::storage::use_storage;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
struct HistoryEntry {
  num: GenericNum,
  base: Base,
  bytes: DataSize,
}

impl IntoView for HistoryEntry {
  fn into_view(self) -> View {
    view! {
      <tr>
        <td>
          {move || base_name(self.base)}
        </td>
        <td>
          {self.num.to_size(self.bytes).to_base(self.base)}
        </td>
        <td>
           {move || size_name(self.bytes)}
        </td>
      </tr>
    }
    .into_view()
  }
}

fn get_history() -> (
  Signal<Vec<HistoryEntry>>,
  WriteSignal<Vec<HistoryEntry>>,
  impl Fn() + Clone,
) {
  use_storage("conversion-history", Vec::<HistoryEntry>::new())
}

fn base_name(base: Base) -> &'static str {
  match base {
    Base::Binary => "BIN",
    Base::Octal => "OCT",
    Base::Decimal => "DEC",
    Base::Hex => "HEX",
  }
}

fn size_name(size: DataSize) -> &'static str {
  match size {
    DataSize::Byte => "BYTE",
    DataSize::Word => "WORD",
    DataSize::DWord => "DWORD",
    _ => "QWORD",
  }
}

#[component]
fn RememberButton(
  read_num: ReadSignal<GenericNum>,
  read_base: ReadSignal<Base>,
) -> impl IntoView {
  let (read_hist, write_hist, _erase_hist) = get_history();
  let remember = move |_| {
    let mut h = read_hist();
    let num = read_num();
    h.push(HistoryEntry {
      num,
      base: read_base(),
      bytes: num.data_size(),
    });
    write_hist(h);
  };
  view! {
    <button type="button" on:click=remember> "Remember" </button>
  }
}

#[component]
fn ClearAllButton() -> impl IntoView {
  let (_, write_hist, _) = get_history();
  view! {
    <button type="button" class="secondary" on:click=move|_| write_hist(vec![])> "Clear history" </button>
  }
}

#[component]
fn History() -> impl IntoView {
  let (read_hist, _write_hist, _erase_hist) = get_history();
  view! {
    <table>
      {move || read_hist()}
    </table>
  }
}

#[component]
pub fn NumberCell(
  read_num: ReadSignal<GenericNum>,
  base: Base,
) -> impl IntoView {
  let get_val = move || {
    read_num()
      .to_base(base)
      .seperate(base.digits_per_byte(), " ")
  };
  view! {
    <td>{get_val}</td>
  }
}

#[component]
pub fn NumberTable(
  read_num: ReadSignal<GenericNum>,
  read_base: ReadSignal<Base>,
  write_base: WriteSignal<Base>,
  read_bytes: ReadSignal<DataSize>,
  write_bytes: WriteSignal<DataSize>,
) -> impl IntoView {
  view! {
    <table role="grid">
      <tr>
        <BaseButton base=Base::Hex read_base write_base/>
        <NumberCell read_num base=Base::Hex/>
      </tr>
      <tr>
        <BaseButton base=Base::Decimal read_base write_base/>
        <NumberCell read_num base=Base::Decimal/>
      </tr>
      <tr>
        <BaseButton base=Base::Octal read_base write_base/>
        <NumberCell read_num base=Base::Octal/>
      </tr>
      <tr>
        <BaseButton base=Base::Binary read_base write_base/>
        <NumberCell read_num base=Base::Binary/>
      </tr>
    </table>
    <table>
      <tr>
        <SizeButton read_bytes write_bytes size=DataSize::Byte/>
        <SizeButton read_bytes write_bytes size=DataSize::Word/>
        <SizeButton read_bytes write_bytes size=DataSize::DWord/>
        <SizeButton read_bytes write_bytes size=DataSize::QWord/>
      </tr>
    </table>
  }
}

#[component]
pub fn SizeButton(
  size: DataSize,
  read_bytes: ReadSignal<DataSize>,
  write_bytes: WriteSignal<DataSize>,
) -> impl IntoView {
  let is_size = move || read_bytes() == size;
  let set_size = move |_| write_bytes(size);
  let name = size_name(size);
  view! {
    <td role="button" type="button" on:click=set_size class="secondary" class:contrast=is_size>
      {name}
    </td>
  }
}

#[component]
pub fn BaseButton(
  base: Base,
  read_base: ReadSignal<Base>,
  write_base: WriteSignal<Base>,
) -> impl IntoView {
  let is_size = move || read_base() == base;
  let set_size = move |_| write_base(base);
  let name = base_name(base);
  view! {
    <td role="button" type="button" on:click=set_size class="secondary" class:contrast=is_size>
      {name}
    </td>
  }
}

#[component]
pub fn BaseConversion() -> impl IntoView {
  let (read_input, write_input) = create_signal("0".to_string());
  let (read_num, write_num) = create_signal(GenericNum::default());
  let (read_bytes, write_bytes) = create_signal(DataSize::Byte);
  let (read_base, write_base) = create_signal(Base::Decimal);

  let write_num_proxy =
    move |n: GenericNum| write_input(n.to_base(read_base()));

  create_effect(move |_| {
    let size = read_bytes();
    let num = read_num();
    write_num(num.to_size(size));
  });

  create_effect(move |_| {
    let _ = read_bytes(); // Ensures effect runs when bytes changes
    let input = read_input();
    let new_num = GenericNum::from_base(&input, read_base());
    match new_num {
      Some(new_num) => write_num(new_num),
      None => {
        write_num(GenericNum::default());
      },
    }
  });

  let get_placeholder = move || match read_base() {
    Base::Binary => "Binary number",
    Base::Octal => "Octal number",
    Base::Decimal => "Decimal number",
    Base::Hex => "Hexadecimal number",
  };

  view! {
    <article>
      <header> "Programming Calculator" </header>
      <div class="grid">
        <div>
          <div>
            <input type="text" placeholder=get_placeholder value=move || read_input() on:input=move|s| {
              let event = event_target_value(&s);
              write_input(event);
            }/>
          </div>
          <NumberTable read_num read_base write_base read_bytes write_bytes/>
        </div>
        <div>
          <ByteTable read_num write_num=write_num_proxy read_bytes/>
        </div>
      </div>
      <hr/>
      <div>
        <RememberButton read_num read_base/>
        <ClearAllButton/>
        <History/>
      </div>
    </article>
  }
}

#[component]
pub fn ByteView<F>(
  read_num: ReadSignal<GenericNum>,
  write_num: F,
  offset: usize,
) -> impl IntoView
where
  F: Fn(GenericNum) + Copy + 'static,
{
  let get_bit = move |n: usize| read_num.get().get_bit(n + offset * 8);
  let flip_bit = move |n: usize| {
    let mut num = read_num.get();
    num.flip_bit(n + offset * 8);
    write_num(num);
  };
  let bit_entry = move |n: usize| {
    view! {
      <td type="button" role="button" class = "secondary" class:contrast=move || get_bit(n) on:click=move|_| flip_bit(n)>
        {move || get_bit(n) as u64}
      </td>
    }
  };
  let entrys = (0..8).rev().map(|n| bit_entry(n)).collect_view();
  view! {
    <tr>
      {entrys}
    </tr>
  }
}

#[component]
pub fn ByteTable<F>(
  read_num: ReadSignal<GenericNum>,
  write_num: F,
  read_bytes: ReadSignal<DataSize>,
) -> impl IntoView
where
  F: Fn(GenericNum) + Copy + 'static,
{
  let table = move || {
    (0..read_bytes() as usize)
      .rev()
      .map(|n| view! {<ByteView read_num write_num offset=n/>})
      .collect_view()
  };
  view! {
    <table class="binary">
      {table}
    </table>
  }
}

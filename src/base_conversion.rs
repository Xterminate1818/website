use based::*;
use leptos::*;
use num_traits::ops::wrapping::WrappingNeg;

#[component]
pub fn ConversionDisplay(
  number: ReadSignal<GenericNum>,
  bytes: ReadSignal<DataSize>,
) -> impl IntoView {
  let get_unsigned = move |base| {
    number
      .get()
      .to_size(bytes.get())
      .to_base(base)
      .seperate(base.digits_per_byte(), " ")
  };
  let get_signed = move |base| {
    let n = number.get().to_size(bytes.get());
    if n.sign_bit() {
      format!(
        "-{}",
        n.wrapping_neg()
          .to_base(base)
          .seperate(base.digits_per_byte(), " ")
      )
    } else {
      format!("+{}", n.to_base(base).seperate(base.digits_per_byte(), " "))
    }
  };

  let get_ubin = move || get_unsigned(Base::Binary);
  let get_udec = move || get_unsigned(Base::Decimal);
  let get_uhex = move || get_unsigned(Base::Hex);

  let get_sbin = move || get_signed(Base::Binary);
  let get_sdec = move || get_signed(Base::Decimal);
  let get_shex = move || get_signed(Base::Hex);

  view! {
    <table>
      <tr>
        <th>""</th><th>"Unsigned"</th><th>"Signed"</th>
      </tr>
      <tr>
        <th>"Decimal"</th> <td>{get_udec}</td> <td>{get_sdec}</td>
      </tr>
      <tr>
        <th>"Hex"</th> <td>{get_uhex}</td> <td>{get_shex}</td>
      </tr>
      <tr>
        <th>"Binary"</th> <td>{get_ubin}</td> <td>{get_sbin}</td>
      </tr>
    </table>
  }
}

#[component]
pub fn BaseSizeSelector(write_bytes: WriteSignal<DataSize>) -> impl IntoView {
  view! {
    <select value="2" on:input=move|s| {
      let event = u32::from_str_radix(&event_target_value(&s), 10).unwrap_or(1);
      write_bytes(match event {
        2 => DataSize::Word,
        4 => DataSize::DWord,
        8 => DataSize::QWord,
        _ => DataSize::Byte,
      });
    }>
      <option value="1">"Byte"</option>
      <option value="2">"Word"</option>
      <option value="4">"Double Word"</option>
      <option value="8">"Quad Word"</option>
    </select>
  }
}

#[component]
pub fn InputFormatSelector(write_base: WriteSignal<Base>) -> impl IntoView {
  view! {
    <select on:input=move|s| {
      let event = u32::from_str_radix(&event_target_value(&s), 10).unwrap_or(1);
      write_base(match event {
        2 => Base::Binary,
        16 => Base::Hex,
        _ => Base::Decimal,
      });
    }>
      <option value="10">"Decimal"</option>
      <option value="16">"Hex"</option>
      <option value="2">"Binary"</option>
    </select>
  }
}

#[component]
pub fn BaseConversion() -> impl IntoView {
  let (read_input, write_input) = create_signal("".to_string());
  let (read_num, write_num) = create_signal(GenericNum::default());
  let (read_bytes, write_bytes) = create_signal(DataSize::Byte);
  let (read_base, write_base) = create_signal(Base::Decimal);

  create_effect(move |_| {
    let _ = read_bytes(); // Ensures effect runs when bytes changes
    let new_num = GenericNum::from_base(&read_input(), read_base());
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
    <div class="calc">
    <h1> Base Conversion </h1>
    <input type="text" placeholder=get_placeholder value=read_input on:input=move|s| {
      let event = event_target_value(&s);
      write_input(event);
    }/>
    <ConversionDisplay number=read_num bytes=read_bytes/>
    <hr/>
    <h3> "Options" </h3>
    <table>
      <tr>
        <td> "Input format:" </td>
        <td> <InputFormatSelector write_base/> </td>
      </tr>
      <tr>
        <td>"Data size: "</td>
        <td> <BaseSizeSelector write_bytes/> </td>
      </tr>
    </table>
    </div>
  }
}

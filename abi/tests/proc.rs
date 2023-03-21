use ethers_abi_enc::{sol, SolType};

use ethers_primitives::{B160, U256};

sol! {
    struct MyStruct {
        uint256 a;
        bytes32 b;
        address[] c;
    }
}

// Works only outside of function scope due to rust import rules
sol! {
    struct MyStruct2 {
        MyStruct a;
        bytes32 b;
        address[] c;
    }
}

// This works
type MyTuple = sol! {
    (MyStruct, bytes32)
};

// This works
type LateBinding<A> = sol! {
    (A[], address)
};

#[test]
fn hello() {
    let a = MyStructRust {
        a: U256::from(1),
        b: [0; 32],
        c: vec![],
    };

    dbg!(MyTuple::hex_encode((a.clone(), [0; 32])));

    dbg!(MyStruct::hex_encode(a.clone()));

    dbg!(LateBinding::<MyStruct>::hex_encode((
        vec![a.clone(), a.clone()],
        B160::default()
    )));

    dbg!(MyStruct2::hex_encode(MyStruct2Rust {
        a,
        b: [0; 32],
        c: vec![],
    }));
}
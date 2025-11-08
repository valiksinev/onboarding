// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract B {
    uint public _b = 0;

    function update_b() public {
        _b++;
    }

    function b() public view returns (uint) {
        return _b;
    }
}

contract A {
    uint public a;
    B public contract_B;

    function deploy_B() public {
        contract_B = new B();
    }

    function update() public {
        a = a + 1;
    }

    function update_b() public {
        require(address(contract_B) != address(0), "B not deployed");
        contract_B.update_b();
    }

    function address_B() public view returns (address) {
        return address(contract_B);
    }
}

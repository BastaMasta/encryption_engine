use std::collections::HashMap;
use rand::Rng;

fn main() {
    let chars_vec : Vec<char> = vec!['0','1','2','3','4','5','6','7','8','9','A','B','C','D',
                                         'E','F','H','I','J','K','L','M','N','O','P','Q','R','S',
                                         'T','U','V','W','X','Y','Z','a','b','c','d','e','f','g',
                                         'h','i','j','k','l','m','n','o','p','q','r','s','t','u',
                                         'v','x','y','z','`','~','!','@','#','$','%','^','&','*',
                                         '(',')','[',']','{','}','-','_','=','+',';',':','\'','"',
                                         ',','<','.','>','/','?','\\','|','¡','¢','£','¤','¥','¦',
                                         '¨','©','ª','«','¬','®','¯','°','±','²','³','´','µ','·',
                                         '¶','¸','¹','º','»','¿','×','ß','æ','ø','þ','Ɔ','Ɣ','ƕ',
                                         'Ɯ','ƛ','Ƨ','Ʃ','ƪ','Ʊ','ƿ','ȸ','ʬ','ʮ','ʪ','ʩ','ʢ','ʡ',
                                         'ʝ'];
    for num in 0..5{
        let mut key_vec = chars_vec.clone();
        let mut val_vec = chars_vec.clone();
        let mut encr_hash : HashMap<char, char> = HashMap::new();
        for i in key_vec {
            let index = rand::thread_rng().gen_range(0..val_vec.len());
            encr_hash.insert(i.clone(), val_vec[index].clone());
            val_vec.remove(index);
        }
        println!("{:?}", encr_hash);
        println!("\n\n\n\n\n\n\n")
    }
}
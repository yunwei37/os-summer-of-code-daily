bool isValid(char * s1){
        char s[100000];
        int i=0,j=0;
        for(j=0;s1[j]!=0;j++)
        {   
            char n = s1[j];
            if (n == '(')                s[i++] = ')';
            else if (n == '{')           s[i++] = '}';
            else if (n == '[')           s[i++] = ']';
            else 
            {
                if( i==0 || s[i-1] != n)      return false; 
                else        i--; 
            }        
        }
        return i==0;
}